import lib.code.data_components
import lib.code.version

if __name__ == "__main__":
    version_id = lib.code.version.get_version_id()
    lib.code.data_components.generate(version_id)
