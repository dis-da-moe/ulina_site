using System;
using UnityEngine;

public class BackButtonStart : MonoBehaviour
{
    AnimationManager animationManager;

    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        if (!animationManager)
        {
            animationManager = GameObject.FindWithTag("AnimationManager").GetComponent<AnimationManager>();
        }
        animationManager.ChangeScene("start");
    }
}
