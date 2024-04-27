package io.github.earthtraveller1.nengpass

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import io.github.earthtraveller1.nengpass.ui.theme.NengPassTheme

class PasswordListActivity : ComponentActivity() {
    private var masterKey: String = ""

    @Composable
    fun TopBar(modifier: Modifier = Modifier) {
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
                    onClick = {},
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

    @Preview
    @Composable
    private fun MainContent(modifier: Modifier = Modifier) {
        NengPassTheme {
            Scaffold(
                topBar = {
                    TopBar(modifier = Modifier)
                },
                modifier = modifier,
            ) { padding ->
                Surface(
                    modifier = modifier
                        .padding(padding)
                        .fillMaxSize(),
                    color = MaterialTheme.colorScheme.background,
                ) {
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