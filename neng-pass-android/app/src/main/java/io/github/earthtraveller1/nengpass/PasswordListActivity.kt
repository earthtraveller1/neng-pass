package io.github.earthtraveller1.nengpass

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.shape.CornerSize
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material3.*
import androidx.compose.material3.ButtonDefaults.buttonColors
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.window.Dialog
import io.github.earthtraveller1.nengpass.ui.theme.NengPassTheme

class PasswordListActivity : ComponentActivity() {
    private var masterKey: String = ""

    @Composable
    fun TopBar(modifier: Modifier = Modifier, setNewPasswordDialog: (Boolean) -> Unit) {
        Surface(
            modifier = modifier.fillMaxWidth(1.0f),
            color = MaterialTheme.colorScheme.primaryContainer,
            contentColor = MaterialTheme.colorScheme.onPrimaryContainer,
        ) {
            Row(
                modifier = modifier.fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    "Your Passwords",
                    modifier = modifier.padding(16.dp),
                    style = MaterialTheme.typography.titleLarge,
                )

                IconButton(
                    onClick = {
                        setNewPasswordDialog(true)
                    },
                    modifier = modifier
                        .padding(24.dp)
                        .size(48.dp)
                ) {
                    Icon(
                        imageVector = Icons.Filled.AddCircle,
                        contentDescription = "Add a password",
                    )
                }
            }
        }
    }

    @Composable
    fun PasswordEntry(modifier: Modifier = Modifier, name: String) {
        Button(
            onClick = {

            },
            colors = buttonColors(
                containerColor = MaterialTheme.colorScheme.secondaryContainer,
                contentColor = MaterialTheme.colorScheme.onSecondaryContainer
            ),
            shape = RoundedCornerShape(corner = CornerSize(10.dp)),
            modifier = modifier
                .padding(horizontal = 18.dp, vertical = 6.dp)
                .fillMaxWidth(),
        ) {
            Row(
                modifier = modifier.fillMaxWidth().padding(vertical = 5.dp),
                verticalAlignment = Alignment.CenterVertically
            ) {
                Icon(
                    imageVector = Icons.Filled.Lock,
                    contentDescription = "A Lock",
                    modifier = modifier.padding(horizontal = 10.dp)
                )

                Text(name)
            }
        }
    }

    @Preview
    @Composable
    private fun MainContent(modifier: Modifier = Modifier) {
        val (createPasswordDialog, setCreatePasswordDialog) = remember { mutableStateOf(false) }
        val (newPasswordName, setNewPasswordName) = remember { mutableStateOf("") }
        val (newPasswordValue, setNewPasswordValue) = remember { mutableStateOf("") }

        val passwordListValue = NengPass.getPasswordList(applicationContext.dataDir.canonicalPath)
        val (passwordList, setPasswordList) = remember { mutableStateOf(passwordListValue) }

        NengPassTheme {
            if (createPasswordDialog) {
                Dialog(
                    onDismissRequest = {
                        setCreatePasswordDialog(false)
                    }
                ) {
                    Surface(modifier = modifier) {
                        Text("Create a password")

                        TextField(
                            value = newPasswordName,
                            onValueChange = { newValue: String ->
                                setNewPasswordName(newValue)
                            },
                            label = {
                                Text("Name")
                            },
                            modifier = modifier
                        )

                        TextField(
                            value = newPasswordValue,
                            onValueChange = { newValue: String ->
                                setNewPasswordValue(newValue)
                            },
                            label = {
                                Text("Password")
                            },
                            modifier = modifier
                        )

                        Button(
                            onClick = {

                            }
                        ) {
                            Text("Create Password")
                        }
                    }
                }
            }

            Scaffold(
                topBar = {
                    TopBar(modifier = Modifier, setCreatePasswordDialog)
                },
                modifier = modifier,
            ) { padding ->
                Surface(
                    modifier = modifier
                        .padding(padding)
                        .fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    Column(
                        modifier = modifier.verticalScroll(rememberScrollState()),
                    ) {
                        for (password in passwordList) {
                            PasswordEntry(modifier = modifier, name = password)
                        }
                    }
                }
            }
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        masterKey = intent.extras!!.getString("masterKey")!!

        setContent {
            MainContent()
        }
    }
}