import lib.code.registry
import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

version_id = lib.code.version.get_version_id()
registries = lib.extract.get_registries_report(version_id)

lib.code.registry.generate_registries(registries)

lib.code.utils.fmt()

print('Done!')
