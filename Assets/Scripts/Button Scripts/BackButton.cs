using System;
using UnityEngine;

public class BackButton : MonoBehaviour
{
    public AnimationManager animationManager;
    
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();

        if (!animationManager)
        {
            animationManager = GameObject.FindWithTag("AnimationManager").GetComponent<AnimationManager>();
        }
        animationManager.ChangeScene("SampleScene");
    }
}
