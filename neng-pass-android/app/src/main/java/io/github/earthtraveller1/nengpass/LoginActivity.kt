package io.github.earthtraveller1.nengpass

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.window.Dialog
import io.github.earthtraveller1.nengpass.ui.theme.NengPassTheme
import java.io.File

class LoginActivity : ComponentActivity() {
    @Composable
    fun ErrorDialog(modifier: Modifier = Modifier, pMessage: String, setNoDialog: (Boolean) -> Unit) {
        Dialog(onDismissRequest = { setNoDialog(false) }) {
            Surface(color = MaterialTheme.colorScheme.error, shape = RoundedCornerShape(16.dp)) {
                Text(
                    pMessage, color = MaterialTheme.colorScheme.onError, modifier = modifier.padding(24.dp)
                )
            }
        }
    }

    @Composable
    fun SetMasterKey(modifier: Modifier = Modifier) {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }
        val (confirmPasswordValue, setConfirmPasswordValue) = remember { mutableStateOf("") }
        val (passwordNoMatchDialog, setPasswordNoMatchDialog) = remember { mutableStateOf(false) }
        val (passwordTooLongDialog, setPasswordTooLongDialog) = remember { mutableStateOf(false) }

        Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = modifier) {
            if (passwordNoMatchDialog) {
                ErrorDialog(modifier, "The passwords that you provided did not match.", setPasswordNoMatchDialog)
            }

            if (passwordTooLongDialog) {
                ErrorDialog(
                    modifier,
                    "The password is too long. Maximum length is 32 characters.",
                    setPasswordTooLongDialog
                )
            }

            Text("Set your master key", fontSize = 6.em, modifier = modifier.padding(top = 24.dp))
            Text("You have not yet set a master key", modifier = modifier.padding(bottom = 48.dp, top = 8.dp))

            NengPass.PasswordField(modifier, "Password", passwordValue, setPasswordValue)
            NengPass.PasswordField(modifier, "Confirm Password", confirmPasswordValue, setConfirmPasswordValue)

            Button(onClick = {
                if (passwordValue != confirmPasswordValue) {
                    setPasswordNoMatchDialog(true)
                    return@Button
                }

                if (passwordValue.length > 32) {
                    setPasswordTooLongDialog(true)
                    return@Button
                }

                NengPass.setMasterKey("${applicationInfo.dataDir}/master_key", passwordValue)

                val intent = Intent(applicationContext, PasswordListActivity::class.java)
                intent.putExtra("masterKey", passwordValue)
                startActivity(intent)
            }) {
                Text("Ok")
            }
        }
    }

    @Composable
    fun Login(modifier: Modifier = Modifier) {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }
        val (passwordIncorrectDialog, setPasswordIncorrectDialog) = remember { mutableStateOf(false) }

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            if (passwordIncorrectDialog) {
                ErrorDialog(modifier, "Sorry, but the password is incorrect", setPasswordIncorrectDialog)
            }

            Text("Login", fontSize = 6.em, modifier = modifier.padding(bottom = 32.dp, top = 16.dp))
            NengPass.PasswordField(modifier, "Password", passwordValue, setPasswordValue)

            Button(onClick = {
                if (NengPass.isMasterKeyCorrect("${applicationInfo.dataDir}/master_key", passwordValue)) {
                    val intent = Intent(applicationContext, PasswordListActivity::class.java)
                    intent.putExtra("masterKey", passwordValue)
                    startActivity(intent)
                } else {
                    setPasswordIncorrectDialog(true)
                }
            }) {
                Text("Login")
            }
        }
    }

    private fun isMasterKeySet(): Boolean {
        val masterKeyFile = File("${applicationInfo.dataDir}/master_key")
        return masterKeyFile.exists()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        NengPass.init()

        setContent {
            NengPassTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(), color = MaterialTheme.colorScheme.background
                ) {
                    if (isMasterKeySet()) {
                        Login()
                    } else {
                        SetMasterKey()
                    }
                }
            }
        }
    }
}
