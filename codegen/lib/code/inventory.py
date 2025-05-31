from typing import Any
from lib.utils import to_camel_case, get_dir_location

# The directory where declare_menus! {} is done
inventory_menus_dir = get_dir_location("../azalea-inventory/src/lib.rs")


def update_menus(initial_menu_entries: dict[str, Any]):
    # new_menus is a dict of { menu_id: { "protocol_id": protocol_id } }
    # so convert that into an array where the protocol id is the index and the
    # values are enum variant names
    new_menus: list[str] = [""] * len(initial_menu_entries)
    for menu_id, menu in initial_menu_entries.items():
        new_menus[menu["protocol_id"]] = menu_name_to_enum_name(menu_id)

    new_menus.insert(0, "Player")

    with open(inventory_menus_dir, "r") as f:
        menus_rs = f.read().splitlines()

        start_line_index = 0

        current_menus = []
        in_the_macro = False
        for i, line in enumerate(menus_rs):
            if line.startswith("declare_menus!"):
                in_the_macro = True
                start_line_index = i
            if in_the_macro:
                if line.startswith("    ") and line.endswith("{"):
                    # get the variant name for this menu
                    current_menu = line[:-1].strip()
                    current_menus.append(current_menu)

        print("current_menus", current_menus)
        print("new_menus", new_menus)

        # now we have the current menus, so compare that with the expected
        # menus and update the file if needed
        if current_menus != new_menus:
            # ok so insert the new menus with todo!() for the body
            current_menus_list_index = 0
            new_menus_list_index = 0
            insert_line_index = start_line_index + 1
            # figure out what menus need to be placed
            while True:
                # if the values at the indexes are the same, add to both and don't do anything
                if (
                    current_menus_list_index < len(current_menus)
                    and new_menus_list_index < len(new_menus)
                    and current_menus[current_menus_list_index]
                    == new_menus[new_menus_list_index]
                ):
                    current_menus_list_index += 1
                    new_menus_list_index += 1
                    # increase insert_line_index until we get a line that starts with }
                    while not menus_rs[insert_line_index].strip().startswith("}"):
                        insert_line_index += 1
                    insert_line_index += 1
                    # print('same', current_menus_list_index,
                    #       new_menus_list_index, insert_line_index)
                # something was added to new_menus but not current_menus
                elif (
                    new_menus_list_index < len(new_menus)
                    and new_menus[new_menus_list_index] not in current_menus
                ):
                    # insert the new menu
                    menus_rs.insert(
                        insert_line_index,
                        f"    {new_menus[new_menus_list_index]} {{\n        todo!()\n    }},",
                    )
                    insert_line_index += 1
                    new_menus_list_index += 1
                    print(
                        "added",
                        current_menus_list_index,
                        new_menus_list_index,
                        insert_line_index,
                    )
                # something was removed from new_menus but is still in current_menus
                elif (
                    current_menus_list_index < len(current_menus)
                    and current_menus[current_menus_list_index] not in new_menus
                ):
                    # remove the current menu
                    while not menus_rs[insert_line_index].strip().startswith("}"):
                        menus_rs.pop(insert_line_index)
                    menus_rs.pop(insert_line_index)
                    current_menus_list_index += 1
                    print(
                        "removed",
                        current_menus_list_index,
                        new_menus_list_index,
                        insert_line_index,
                    )

                # if current_menus_list_index overflowed, then add the rest of the new menus
                elif current_menus_list_index >= len(current_menus):
                    for i in range(new_menus_list_index, len(new_menus)):
                        menus_rs.insert(
                            insert_line_index,
                            f"    {new_menus[i]} {{\n        todo!()\n    }},",
                        )
                        insert_line_index += 1
                    print(
                        "current_menus_list_index overflowed",
                        current_menus_list_index,
                        new_menus_list_index,
                        insert_line_index,
                    )
                    break
                # if new_menus_list_index overflowed, then remove the rest of the current menus
                elif new_menus_list_index >= len(new_menus):
                    for _ in range(current_menus_list_index, len(current_menus)):
                        while not menus_rs[insert_line_index].strip().startswith("}"):
                            menus_rs.pop(insert_line_index)
                        menus_rs.pop(insert_line_index)
                        # current_menus_list_index += 1
                    print(
                        "new_menus_list_index overflowed",
                        current_menus_list_index,
                        new_menus_list_index,
                        insert_line_index,
                    )
                    break
    with open(inventory_menus_dir, "w") as f:
        f.write("\n".join(menus_rs))


def menu_name_to_enum_name(menu_name: str) -> str:
    return to_camel_case(menu_name.split(":")[-1])
