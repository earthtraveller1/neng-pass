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
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.platform.LocalClipboardManager
import androidx.compose.ui.text.AnnotatedString
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
                    onClick = { setNewPasswordDialog(true) },
                    modifier = modifier.padding(24.dp).size(48.dp)
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
    fun PasswordEntry(modifier: Modifier = Modifier, name: String, onClick: () -> Unit = {}) {
        Button(
            onClick = onClick,
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

    @Composable
    fun BaseDialogSurface(modifier: Modifier = Modifier, color: Color, content: @Composable ColumnScope.() -> Unit) {
        Surface(
            modifier = modifier,
            color = color,
            shape = RoundedCornerShape(16.dp)
        ) {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = modifier.padding(24.dp),
                content = content
            )
        }
    }

    @Composable
    fun DeletePasswordDialog(
        modifier: Modifier = Modifier,
        passwordName: String,
        setPasswordDialog: (Boolean) -> Unit,
        setDeletePasswordDialog: (Boolean) -> Unit,
        setPasswordList: (Array<String>) -> Unit
    ) {
        Dialog(
            onDismissRequest = {
                setPasswordDialog(true)
                setDeletePasswordDialog(false)
            }
        ) {
            BaseDialogSurface(modifier, MaterialTheme.colorScheme.errorContainer) {
                Text("Do you want to delete $passwordName?")

                Row(modifier = modifier.padding(top = 16.dp)) {
                    Button(
                        onClick = {
                            setPasswordDialog(true)
                            setDeletePasswordDialog(false)
                        },
                        colors = buttonColors(
                            containerColor = Color.DarkGray,
                            contentColor = Color.LightGray
                        ),
                        modifier = modifier.padding(horizontal = 16.dp)
                    ) {
                        Text("No")
                    }

                    Button(
                        onClick = {
                            NengPass.deletePassword(applicationContext.dataDir.canonicalPath, passwordName)
                            setPasswordList(NengPass.getPasswordList(applicationContext.dataDir.canonicalPath))
                            setDeletePasswordDialog(false)
                        },
                        colors = buttonColors(
                            containerColor = MaterialTheme.colorScheme.onErrorContainer,
                            contentColor = MaterialTheme.colorScheme.errorContainer
                        ),
                        modifier = modifier.padding(horizontal = 16.dp)
                    ) {
                        Text("Yes")
                    }
                }
            }
        }
    }

    @Composable
    fun PasswordDialog(
        modifier: Modifier = Modifier,
        setPasswordDialog: (Boolean) -> Unit,
        setDeletePasswordDialog: (Boolean) -> Unit,
        passwordName: String
    ) {
        val password = NengPass.getPassword(applicationContext.dataDir.canonicalPath, masterKey, passwordName)
        val (isPasswordVisible, setIsPasswordVisible) = remember { mutableStateOf(false) }

        Dialog(onDismissRequest = { setPasswordDialog(false) }) {
            BaseDialogSurface(modifier, MaterialTheme.colorScheme.secondaryContainer) {
                Text(
                    text = passwordName,
                    style = MaterialTheme.typography.titleLarge,
                    color = MaterialTheme.colorScheme.onSecondaryContainer
                )

                if (isPasswordVisible) {
                    Text(
                        password,
                        color = MaterialTheme.colorScheme.onSecondaryContainer,
                        modifier = modifier.padding(vertical = 16.dp)
                    )
                } else {
                    val hiddenPasswordBuilder = StringBuilder()

                    for (i in 1..(password.length)) {
                        hiddenPasswordBuilder.append('*')
                    }

                    val hiddenPassword = hiddenPasswordBuilder.toString()

                    Text(
                        hiddenPassword,
                        modifier = modifier.padding(vertical = 16.dp),
                        color = MaterialTheme.colorScheme.onSecondaryContainer
                    )
                }

                Button(
                    onClick = { setIsPasswordVisible(!isPasswordVisible) },
                    modifier = modifier.padding(top = 16.dp),
                ) {
                    if (isPasswordVisible) {
                        Text("Hide Password")
                    } else {
                        Text("Show Password")
                    }
                }

                val clipboard = LocalClipboardManager.current

                Button(
                    onClick = { clipboard.setText(AnnotatedString(password)) },
                    modifier = modifier.padding(top = 8.dp)
                ) {
                    Text("Copy to Clipboard!")
                }

                Button(
                    onClick = {
                        setDeletePasswordDialog(true)
                        setPasswordDialog(false)
                    },
                    modifier = modifier.padding(top = 8.dp),
                    colors = buttonColors(
                        containerColor = MaterialTheme.colorScheme.error,
                        contentColor = MaterialTheme.colorScheme.onError,
                    )
                ) {
                    Text("Delete Password")
                }
            }
        }
    }

    @Composable
    fun CreatePasswordDialog(
        modifier: Modifier = Modifier,
        setDialog: (Boolean) -> Unit,
        setPasswordList: (Array<String>) -> Unit
    ) {
        val (newPasswordName, setNewPasswordName) = remember { mutableStateOf("") }
        val (newPasswordValue, setNewPasswordValue) = remember { mutableStateOf("") }
        val (newPasswordError, setNewPasswordError) = remember { mutableStateOf("") }

        Dialog(onDismissRequest = { setDialog(false) }) {
            BaseDialogSurface(modifier, MaterialTheme.colorScheme.secondaryContainer) {
                Text("Create a password", color = MaterialTheme.colorScheme.onSecondaryContainer)

                TextField(
                    value = newPasswordName,
                    onValueChange = { newValue: String -> setNewPasswordName(newValue) },
                    label = { Text("Name") },
                    modifier = modifier.padding(vertical = 16.dp)
                )

                NengPass.PasswordField(modifier, "Password", newPasswordValue, setNewPasswordValue)

                if (newPasswordError != "") {
                    Text(newPasswordError, color = MaterialTheme.colorScheme.error)
                }

                Button(
                    onClick = {
                        if (newPasswordValue.length > 16) {
                            setNewPasswordError("Your password is too long. Maximum length is 16")
                        } else if (newPasswordName == "") {
                            setNewPasswordError("Password must have a name")
                        } else if (newPasswordValue == "") {
                            setNewPasswordError("You didn't enter your password")
                        } else {
                            setNewPasswordError("")
                            NengPass.savePassword(
                                applicationContext.dataDir.canonicalPath,
                                masterKey,
                                newPasswordName,
                                newPasswordValue
                            )
                            setNewPasswordName("")
                            setNewPasswordValue("")

                            // Refresh the password list.
                            val newPasswordList = NengPass.getPasswordList(applicationContext.dataDir.canonicalPath)
                            setPasswordList(newPasswordList)

                            setDialog(false)
                        }
                    },
                    modifier = modifier.padding(vertical = 16.dp)
                ) {
                    Text("Create Password")
                }

                Button(
                    onClick = { setNewPasswordValue(NengPass.generatePassword()) },
                    modifier = modifier.padding(vertical = 16.dp)
                ) {
                    Text("Generate Random Password")
                }
            }
        }
    }

    @Composable
    fun MainContent(modifier: Modifier = Modifier) {
        val (createPasswordDialog, setCreatePasswordDialog) = remember { mutableStateOf(false) }
        val (passwordDialog, setPasswordDialog) = remember { mutableStateOf(false) }
        val (deletePasswordDialog, setDeletePasswordDialog) = remember { mutableStateOf(false) }

        val (currentPasswordName, setCurrentPasswordName) = remember { mutableStateOf("") }

        val passwordListValue = NengPass.getPasswordList(applicationContext.dataDir.canonicalPath)
        val (passwordList, setPasswordList) = remember { mutableStateOf(passwordListValue) }

        NengPassTheme {
            if (createPasswordDialog) {
                CreatePasswordDialog(modifier, setCreatePasswordDialog, setPasswordList)
            }

            if (passwordDialog) {
                PasswordDialog(modifier = modifier, setPasswordDialog, setDeletePasswordDialog, currentPasswordName)
            }

            if (deletePasswordDialog) {
                DeletePasswordDialog(
                    modifier = modifier,
                    currentPasswordName,
                    setPasswordDialog,
                    setDeletePasswordDialog,
                    setPasswordList
                )
            }

            Scaffold(
                topBar = { TopBar(modifier = Modifier, setCreatePasswordDialog) },
                modifier = modifier,
            ) { padding ->
                Surface(
                    modifier = modifier.padding(padding).fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
                    Column(
                        modifier = modifier
                            .verticalScroll(rememberScrollState())
                            .padding(top = 16.dp),
                    ) {
                        for (password in passwordList) {
                            if (password.trim() != "") {
                                PasswordEntry(modifier = modifier, name = password, onClick = {
                                    setCurrentPasswordName(password)
                                    setPasswordDialog(true)
                                })
                            }
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