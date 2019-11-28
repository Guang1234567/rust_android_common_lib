package com.rust.example.android

import android.nfc.Tag
import android.os.Bundle
import android.util.Log
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
        sample_text.text =
            stringFromJNI() + "\n\n" + greeting("world") + "\n\n" + rustSqlite(getDatabasePath("rust_sqlite_demo.db").absolutePath)

        asyncCallBack(this)

        val outputByte: ByteArray = greetingByte("byte".toByteArray())
        //Log.i(TAG, outputByte.toString())
    }

    /**
     * A native method that is implemented by the 'native-lib' native library,
     * which is packaged with this application.
     */
    private external fun stringFromJNI(): String

    private external fun greeting(input: String): String

    private external fun greetingByte(input: ByteArray): ByteArray

    private external fun rustSqlite(dbPath: String): String

    private external fun asyncCallBack(cb: MainActivity): Unit

    private fun onCallBack() : String {
        Thread.sleep(3000)
        Log.w(TAG, "onCallBack: thread id = " + Thread.currentThread().id +
        ", thread name = " + Thread.currentThread().name)

        return "oop!!!"
    }

    companion object {

        const val TAG: String = "MainActivity"

        // Used to load the 'native-lib' library on application startup.
        init {
            // <= 5.0
            System.loadLibrary("greetings")
            // >= 6.0
            System.loadLibrary("native-lib")
        }
    }
}
