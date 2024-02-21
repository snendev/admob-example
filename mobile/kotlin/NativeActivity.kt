package dev.snen.adexample

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import com.google.android.gms.ads.MobileAds

const val AD_ACTIVITY_REQUEST_CODE = 1

class NativeActivity(): android.app.NativeActivity() {
    private var _earned_reward = false
    private var _canceled = false

    override fun onCreate(savedInstanceState: Bundle?) {
        println("we are being recreated for some reason!!!!")
        super.onCreate(savedInstanceState)
        MobileAds.initialize(this) {}
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, resultData: Intent?) {
        super.onActivityResult(requestCode, resultCode, resultData)
        if (requestCode != AD_ACTIVITY_REQUEST_CODE) {
            return
        }
        if (resultCode == Activity.RESULT_OK) {
            _earned_reward = true
        } else if (resultCode == Activity.RESULT_CANCELED) {
            println("Result code is a canceled result!")
            if (resultData != null) {
                println(resultData.getBooleanExtra("dev.snen.adexample.ReceivedReward", false))
            }
            _canceled = true
        }
    }

    fun startAdActivity() {
        val intent = Intent(this, dev.snen.adexample.AdActivity::class.java)
        startActivityForResult(intent, AD_ACTIVITY_REQUEST_CODE)
        _earned_reward = false
        _canceled = false
    }

    fun didEarnReward(): Boolean {
        return _earned_reward
    }

    fun didCancel(): Boolean {
        return _canceled
    }
}
