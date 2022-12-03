# The directoey where declare_menus! {} is done
inventory_menus_dir = get_dir_location(f'../azalea-inventory/src/lib.rs')

def update_menus():
    with open(inventory_menus_dir, 'r') as f:
        menus_rs = f.read().splitlines()

        current_menus = []

        in_the_macro = False
        for line in menus_rs:
            if line.startswith('declare_menus!'):
                in_the_macro = True
            if in_the_macro:
                if line.startswith('    ') and line.endswith('{'):
                    current_menus.append(line.)

