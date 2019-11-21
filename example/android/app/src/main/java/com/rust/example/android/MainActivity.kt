package com.rust.example.android

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }

    override fun onResume() {
        super.onResume()

        // Example of a call to a native method
        sample_text.text = stringFromJNI() + greeting("world")

        rustSqlite(getDatabasePath("rust_sqlite_demo.db").absolutePath)
    }

    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    external fun stringFromJNI(): String

    external fun greeting(pattern: String): String

    external fun rustSqlite(dbPath: String): String

    companion object {

        // Used to load the 'native-lib' library on application startup.
        init {
            // <= 5.0
            System.loadLibrary("greetings")
            // >= 6.0
            System.loadLibrary("native-lib")
        }
    }
}
