package io.github.earthtraveller1.nengpass

interface NengPass {
    companion object {
        init {
            System.loadLibrary("neng_pass_android")
        }

        external fun setMasterKey(file: String, newMasterKey: String)

        external fun isMasterKeyCorrect(file: String, masterKey: String): Boolean
    }
}

