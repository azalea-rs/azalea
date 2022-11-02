import lib.code.language
import lib.code.version
import lib.code.utils
import lib.download
import lib.extract
import lib.utils

version_id = lib.code.version.get_version_id()
language = lib.extract.get_en_us_lang(version_id)

lib.code.language.write_language(language)

print('Done!')
