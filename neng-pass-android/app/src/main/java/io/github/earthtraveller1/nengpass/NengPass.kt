package io.github.earthtraveller1.nengpass

import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp

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

        @Composable
        public fun PasswordField(
            modifier: Modifier = Modifier, label: String, value: String, setValue: (String) -> Unit
        ) {
            TextField(
                value = value,
                label = { Text(label) },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setValue(it) },
                keyboardOptions = KeyboardOptions(
                    keyboardType = KeyboardType.Password,
                    capitalization = KeyboardCapitalization.None,
                    autoCorrect = false
                ),
                modifier = modifier.padding(vertical = 24.dp)
            )
        }
    }
}

