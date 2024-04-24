package io.github.earthtraveller1.nengpass

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.KeyboardCapitalization
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import androidx.compose.ui.window.Dialog
import io.github.earthtraveller1.nengpass.ui.theme.NengPassTheme
import java.io.File

class LoginActivity : ComponentActivity() {
    @Composable
    fun SetMasterKey(modifier: Modifier = Modifier) {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }
        val (confirmPasswordValue, setConfirmPasswordValue) = remember { mutableStateOf("") }
        val (passwordNoMatchDialog, setPasswordNoMatchDialog) = remember { mutableStateOf(false) }

        Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = modifier) {
            if (passwordNoMatchDialog) {
                Dialog(
                    onDismissRequest = { setPasswordNoMatchDialog(false) }
                ) {
                    Text("The passwords that you provided did not match.")
                }
            }

            Text("Set your master key", fontSize = 6.em, modifier = modifier.padding(top = 24.dp))
            Text("You have not yet set a master key", modifier = modifier.padding(bottom = 48.dp, top = 8.dp))

            TextField(
                value = passwordValue,
                label = { Text("Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setPasswordValue(it) },
                keyboardOptions = KeyboardOptions(
                    capitalization = KeyboardCapitalization.None,
                    autoCorrect = false,
                ),
                modifier = modifier.padding(vertical = 24.dp)
            )

            TextField(
                value = confirmPasswordValue,
                label = { Text("Confirm Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setConfirmPasswordValue(it) },
                modifier = modifier.padding(bottom = 24.dp)
            )

            Button(
                onClick = {
                    if (passwordValue == confirmPasswordValue) {
                        NengPass.setMasterKey("${applicationInfo.dataDir}/master_key", passwordValue)

                        val intent = Intent(applicationContext, PasswordListActivity::class.java)
                        startActivity(intent)
                    } else {
                        setPasswordNoMatchDialog(true)
                    }
                }
            ) {
                Text("Ok")
            }
        }
    }

    @Composable
    fun Login() {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }
        val (passwordIncorrectDialog, setPasswordIncorrectDialog) = remember { mutableStateOf(false) }

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            if (passwordIncorrectDialog) {
                Dialog (
                    onDismissRequest = { setPasswordIncorrectDialog(false) }
                ) {
                    Text("Sorry, but the password is incorrect.")
                }
            }

            Text("Login", fontSize = 6.em)
            TextField(
                value = passwordValue,
                label = { Text("Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setPasswordValue(it) }
            )

            Button(
                onClick = {
                    if (NengPass.isMasterKeyCorrect("${applicationInfo.dataDir}/master_key", passwordValue)) {
                        val intent = Intent(applicationContext, PasswordListActivity::class.java)
                        startActivity(intent)
                    } else {
                        setPasswordIncorrectDialog(true)
                    }
                }
            ) {
                Text("Login")
            }
        }
    }

    fun isMasterKeySet(): Boolean {
        val masterKeyFile = File("${applicationInfo.dataDir}/master_key")
        return masterKeyFile.exists()
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        setContent {
            NengPassTheme {
                // A surface container using the 'background' color from the theme
                Surface(
                    modifier = Modifier.fillMaxSize(),
                    color = MaterialTheme.colorScheme.background
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
