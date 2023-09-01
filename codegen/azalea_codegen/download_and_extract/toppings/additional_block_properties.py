from burger.toppings.topping import Topping
from burger.util import WalkerCallback, walk_method
from jawa.cf import ClassFile
from jawa.classloader import ClassLoader
from jawa.constants import FieldReference
from jawa.methods import Method
from jawa.util.descriptor import method_descriptor

from azalea_codegen.utils import bytecode_utils


def _find_friction_setter(block_properties_class: ClassFile) -> Method:
    # Look through the disassembly of the BlockBehaviour.Properties' <init> method to find the `friction` field.
    found_friction_field = False
    friction_field: FieldReference | None = None

    for insn in block_properties_class.methods.find_one(name='<init>').code.disassemble():
        if insn.mnemonic == 'ldc' and abs(insn.operands[0].value - 0.6) < 1e-5:
            found_friction_field = True

        if found_friction_field and insn.mnemonic == 'putfield':
            friction_field = insn.operands[0]
            break

    if friction_field is None:
        raise Exception('Couldn\'t find friction field in BlockBehaviour.Properties')

    # Search for the setter.
    friction_setter = None

    for method in block_properties_class.methods.find(args='F'):
        for insn in method.code.disassemble():
            if insn.mnemonic == 'putfield' and bytecode_utils.field_reference_eq(insn.operands[0], friction_field):
                friction_setter = method
                break

        if friction_setter is not None:
            break

    if friction_setter is None:
        raise Exception('Couldn\'t find friction setter in BlockBehaviour.Properties')

    return friction_setter


def _find_correct_tool_setter(classloader: ClassLoader, block_behaviour_class: ClassFile) -> Method:
    # Find the method `public float getDestroyProgress(BlockState lvt1, Player lvt2, BlockGetter lvt3, BlockPos lvt4)`
    destroy_progress_candidates = []

    for method in block_behaviour_class.methods.find(returns='F'):
        desc = method_descriptor(method.descriptor.value)

        if len(desc.args) == 4 and all([arg.base_type == 'L' for arg in desc.args]):
            destroy_progress_candidates.append(method)

    if len(destroy_progress_candidates) == 0:
        raise Exception('Failed to find BlockBehaviour#getDestroyProgress')

    if len(destroy_progress_candidates) >= 2:
        raise Exception('Found multiple candidates for BlockBehaviour#getDestroyProgress')

    # Now we disassemble the method and search for a method call against the second argument.
    get_destroy_progress = destroy_progress_candidates[0]
    get_destroy_progress_desc = method_descriptor(get_destroy_progress.descriptor.value)

    # Second argument is an instance of Player.
    player_name = get_destroy_progress_desc.args[1].name
    player_has_correct_tool = None

    for insn in get_destroy_progress.code.disassemble():
        if insn.mnemonic != 'invokevirtual':
            continue

        target_method = insn.operands[0]

        if target_method.class_.name.value == player_name:
            (_, player_has_correct_tool) = bytecode_utils.find_called_method(classloader, target_method)
            break

    if player_has_correct_tool is None:
        raise Exception('Failed to find Player#hasCorrectToolForDrops')

    # Disassemble hasCorrectToolForDrops to find the call to `BlockState#requiresCorrectToolForDrops`
    bsb_requires_correct_tool: Method | None = None
    bsb: ClassFile | None = None

    for insn in player_has_correct_tool.code.disassemble():
        if insn.mnemonic == 'invokevirtual':
            # First method call is the correct one.
            (bsb, bsb_requires_correct_tool) = bytecode_utils.find_called_method(classloader, insn.operands[0])

            break

    if bsb_requires_correct_tool is None:
        raise Exception('Failed to find BlockState#requiresCorrectToolForDrops')

    # Disassemble requiresCorrectToolForDrops to find the field in BlockStateBase
    bsb_requires_correct_tool_field = None

    for insn in bsb_requires_correct_tool.code.disassemble():
        if insn.mnemonic == 'getfield':
            bsb_requires_correct_tool_field = insn.operands[0]
            break

    if bsb_requires_correct_tool_field is None:
        raise Exception('Failed to find BlockStateBase#requiresCorrectToolForDrops')

    # Disassemble the initializer of BlockStateBase.
    bp_requires_correct_tool_field: FieldReference | None = None
    block_properties_class: ClassFile | None = None
    last_get_field: FieldReference | None = None

    for insn in bsb.methods.find_one(name='<init>').code.disassemble():
        if insn.mnemonic == 'getfield':
            last_get_field = insn.operands[0]

        if insn.mnemonic == 'putfield':
            if insn.operands[0] == bsb_requires_correct_tool_field:
                assert last_get_field is not None
                (block_properties_class, _) = bytecode_utils.find_accessed_field(classloader, last_get_field)
                bp_requires_correct_tool_field = last_get_field

                break

    if bp_requires_correct_tool_field is None:
        raise Exception('Failed to find field BlockBehaviour.Properties#requiresCorrectToolForDrops')

    # Find the matching setter.
    bp_setter = None

    for method in block_properties_class.methods:
        if len(method.args) != 0:
            continue

        for insn in method.code.disassemble():
            if insn.mnemonic == 'putfield' and bytecode_utils.field_reference_eq(insn.operands[0], bp_requires_correct_tool_field):
                bp_setter = method
                break

        if bp_setter is not None:
            break

    if bp_setter is None:
        raise Exception('Failed to find method BlockBehaviour.Properties#requiresCorrectToolForDrops')

    return bp_setter


class AdditionalPropertyExtractor(Topping):
    """Extracts additional properties from blocks (namely, the friction for the block and whether it requires a tool
    to drop items)"""
    PROVIDES = [
        'blocks.friction',
        'blocks.requires_correct_tool_for_drops'
    ]

    DEPENDS = [
        'identify.block.superclass',
        'identify.block.register',
        'identify.block.list',
        'identify.identifier',
        'blocks',
        'language',
        'version.data',
        'version.is_flattened'
    ]

    @staticmethod
    def act(aggregate, classloader: ClassLoader, verbose=False):
        blocks = aggregate["blocks"]
        block = blocks["block"]
        block_fields = blocks["block_fields"]

        # Get required classes.
        blocks_name = aggregate["classes"]["block.list"]
        blocks_class = classloader[blocks_name]
        block_name = aggregate["classes"]["block.superclass"]
        block_class = classloader[block_name]
        block_ctor = block_class.methods.find_one(name="<init>")
        block_properties_name = block_ctor.args[0].name
        block_properties_class = classloader[block_properties_name]

        # Locate BlockBehaviour by getting it from BlockBehaviour.Properties
        block_behaviour_name = block_properties_name.split('$')[0]
        block_behaviour_class = classloader[block_behaviour_name]

        # Scan for required methods.
        friction_setter = _find_friction_setter(block_properties_class)
        print(friction_setter.name.value, friction_setter.descriptor.value)

        requires_correct_tool_setter = _find_correct_tool_setter(classloader, block_behaviour_class)

        class Walker(WalkerCallback):
            # This walker is mostly modified from the one in Burger's standard `blocks.py` topping.
            def __init__(self):
                self.cur_id = 0

            def on_new(self, ins, const):
                class_name = const.name.value
                return {
                    'class': class_name,
                    'requires_correct_tool_for_drops': False,
                }

            def on_invoke(self, ins, const, obj, args):
                method_name = const.name_and_type.name.value
                method_desc = const.name_and_type.descriptor.value
                desc = method_descriptor(method_desc)

                if ins.mnemonic == "invokestatic":
                    if const.class_.name.value == blocks_name:
                        if len(desc.args) == 2 and desc.args[0].name == "java/lang/String" and \
                                desc.args[1].name == block_name:
                            # Call to the static register method.
                            text_id = args[0]
                            current_block = args[1]
                            block[text_id].update(current_block)

                            return current_block

                        else:
                            # In 20w12a+ (1.16), some blocks (e.g. logs) use a separate method
                            # for initialization.  Call them.
                            sub_method = blocks_class.methods.find_one(name=method_name, args=desc.args_descriptor,
                                                                       returns=desc.returns_descriptor)
                            return walk_method(blocks_class, sub_method, self, verbose, args)

                    return {}

                else:
                    if method_name == "hasNext":
                        # We've reached the end of block registration
                        # (and have started iterating over registry keys)
                        raise StopIteration()

                    elif method_name == friction_setter.name.value and method_desc == friction_setter.descriptor.value:
                        # Call to the friction setter.
                        obj['friction'] = args[0]

                    elif method_name == requires_correct_tool_setter.name.value and method_desc == requires_correct_tool_setter.descriptor.value:
                        # Call to the requires correct tool setter.
                        obj['requires_correct_tool_for_drops'] = True

                    elif method_name == "<init>":
                        # Call to the constructor for the block
                        # The majority of blocks have a 1-arg constructor simply taking the builder.
                        # However, sand has public BlockSand(int color, Block.Builder builder), and
                        # signs (as of 1.15-pre1) have public BlockSign(Block.builder builder, WoodType type)
                        # (Prior to that 1.15-pre1, we were able to assume that the last argument was the builder)
                        # There are also cases of arg-less constructors, which we just ignore as they are presumably not
                        # blocks.
                        for idx, arg in enumerate(desc.args):
                            if arg.name == block_properties_name:
                                obj.update(args[idx])
                                break

                    if desc.returns.name == block_properties_name or desc.returns.name == block_name:
                        return obj

                    elif desc.returns.name == aggregate["classes"]["identifier"]:
                        # Probably getting the air identifier from the registry
                        return "air"

                    elif desc.returns.name != "void":
                        return object()

            def on_get_field(self, ins, const, obj):
                if const.class_.name.value == block_name:
                    # Probably getting the static AIR resource location
                    return "air"

                elif const.class_.name.value == blocks_name:
                    return block[block_fields[const.name_and_type.name.value]]

                elif const.name_and_type.descriptor == "Ljava/util/function/ToIntFunction;":
                    # Light level lambda, used by candles.  Not something we
                    # can evaluate (it depends on the block state).
                    return None

                else:
                    return object()

            def on_put_field(self, ins, const, obj, value):
                pass

            def on_invokedynamic(self, ins, const, args):
                # 1.15-pre2 introduced a Supplier<BlockEntityType> parameter,
                # and while most blocks handled it in their own constructor,
                # chests put it directly in initialization.  We don't care about
                # the value (we get block entities in a different way), but
                # we still need to override this as the default implementation
                # raises an exception

                # 20w12a changed light levels to use a lambda, and we do
                # care about those.  The light level is a ToIntFunction<BlockState>.
                method_desc = const.name_and_type.descriptor.value
                desc = method_descriptor(method_desc)
                if desc.returns.name == "java/util/function/ToIntFunction":
                    return None
                else:
                    return object()

        # Find the static block registration method and walk it.
        blocks_clinit = blocks_class.methods.find_one(name='<clinit>')
        walk_method(blocks_class, blocks_clinit, Walker(), verbose)
