package io.github.earthtraveller1.nengpass

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.em

import java.io.File

import io.github.earthtraveller1.nengpass.ui.theme.NengPassTheme

class LoginActivity : ComponentActivity() {
    @Composable
    fun SetMasterKey(modifier: Modifier = Modifier) {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }
        val (confirmPasswordValue, setConfirmPasswordValue) = remember { mutableStateOf("") }

        Column(horizontalAlignment = Alignment.CenterHorizontally, modifier = modifier) {
            Text("Set your master key", fontSize = 6.em, modifier = modifier)
            Text("You have not yet set a master key", modifier = modifier)

            TextField(
                value = passwordValue,
                label = { Text("Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setPasswordValue(it) },
                modifier = modifier
            )

            TextField(
                value = confirmPasswordValue,
                label = { Text("Confirm Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setConfirmPasswordValue(it) },
                modifier = modifier
            )

            Button(
                onClick = {
                    /* TODO: Set master key */
                }
            ) {
                Text("Ok")
            }
        }
    }

    @Composable
    fun Login() {
        val (passwordValue, setPasswordValue) = remember { mutableStateOf("") }

        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Text("Login", fontSize = 6.em)
            TextField(
                value = passwordValue,
                label = { Text("Password") },
                visualTransformation = PasswordVisualTransformation(),
                onValueChange = { setPasswordValue(it) }
            )

            Button(
                onClick = {
                    /* TODO: Login */
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
