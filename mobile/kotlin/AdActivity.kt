package dev.snen.adexample

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.util.Log
import com.google.android.gms.ads.AdError
import com.google.android.gms.ads.AdRequest
import com.google.android.gms.ads.FullScreenContentCallback
import com.google.android.gms.ads.LoadAdError
import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAd
import com.google.android.gms.ads.rewardedinterstitial.RewardedInterstitialAdLoadCallback

private const val AD_UNIT_ID = "ca-app-pub-3940256099942544/5354046379"
private const val TAG = "AdActivity"

class AdActivity(): Activity() {

    private var rewardedAd: RewardedInterstitialAd? = null
    private var userReceivedReward = false

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        var adRequest = AdRequest.Builder().build()
        RewardedInterstitialAd.load(
            this,
            AD_UNIT_ID,
            adRequest,
            object : RewardedInterstitialAdLoadCallback() {
                override fun onAdFailedToLoad(adError: LoadAdError) {
                    super.onAdFailedToLoad(adError)
                    Log.d(TAG, adError.toString())
                    rewardedAd = null
                }

                override fun onAdLoaded(ad: RewardedInterstitialAd) {
                    super.onAdLoaded(ad)
                    Log.d(TAG, "Ad was loaded.")
                    rewardedAd = ad
                    showRewardedVideo()
                }
            }
        )
    }

    private fun showRewardedVideo() {
        if (rewardedAd == null) {
            Log.d(TAG, "The rewarded interstitial ad wasn't ready yet.")
            return
        }

        val context = this

        rewardedAd!!.fullScreenContentCallback =
            object : FullScreenContentCallback() {
                override fun onAdDismissedFullScreenContent() {
                    Log.d(TAG, "Ad was dismissed.")

                    // Don't forget to set the ad reference to null so you
                    // don't show the ad a second time.
                    rewardedAd = null

                    // todo borrow this class from googleads-mobile-android-examples
                    // if (googleMobileAdsConsentManager.canRequestAds) {
                    //    // Preload the next rewarded interstitial ad.
                    //    loadRewardedAd()
                    // }

                    val intent = Intent(context, dev.snen.adexample.NativeActivity::class.java)
                    Log.d(TAG, "What the: ")
                    Log.d(TAG, userReceivedReward.toString())
                    intent.setFlags(Intent.FLAG_ACTIVITY_CLEAR_TOP or Intent.FLAG_ACTIVITY_SINGLE_TOP)
                    intent.putExtra("dev.snen.adexample.ReceivedReward", userReceivedReward)
                    if (userReceivedReward) {
                        Log.d(TAG, "Result should be OK!")
                        setResult(Activity.RESULT_OK, intent)
                    } else {
                        Log.d(TAG, "Result should be Canceled!")
                        setResult(Activity.RESULT_CANCELED, intent)
                    }
                    finish()
                }

                override fun onAdFailedToShowFullScreenContent(adError: AdError) {
                    Log.d(TAG, "Ad failed to show.")

                    // Don't forget to set the ad reference to null so you
                    // don't show the ad a second time.
                    rewardedAd = null
                }

                override fun onAdShowedFullScreenContent() {
                    Log.d(TAG, "Ad showed fullscreen content.")
                }
            }


        rewardedAd?.show(this) { rewardItem ->
            Log.d(TAG, "User earned the reward.")
            userReceivedReward = true
        }
    }    
}
