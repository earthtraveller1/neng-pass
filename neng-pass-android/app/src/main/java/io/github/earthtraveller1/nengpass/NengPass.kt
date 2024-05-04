package io.github.earthtraveller1.nengpass

interface NengPass {
    companion object {
        init {
            System.loadLibrary("neng_pass_android")
        }

        external fun init()

        external fun setMasterKey(file: String, newMasterKey: String)

        external fun isMasterKeyCorrect(file: String, masterKey: String): Boolean

        external fun getPasswordList(databaseFile: String): Array<String>

        external fun generatePassword(): String

        external fun savePassword(databaseFile: String, masterKey: String, name: String, password: String)

        external fun getPassword(databaseFile: String, masterKey: String, name: String): String

        external fun deletePassword(databaseFile: String, name: String)
    }
}

