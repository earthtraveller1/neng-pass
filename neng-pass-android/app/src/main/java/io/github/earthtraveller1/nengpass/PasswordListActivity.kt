package io.github.earthtraveller1.nengpass

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.material3.Text

class PasswordListActivity: ComponentActivity() {
    private var masterKey: String = ""

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        masterKey = intent.extras!!.getString("masterKey")!!

        setContent {
            Text("The master key appears to be $masterKey")
        }
    }
}