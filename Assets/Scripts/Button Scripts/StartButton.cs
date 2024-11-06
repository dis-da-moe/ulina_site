using System;
using UnityEngine;

public class StartButton : MonoBehaviour
{
     AnimationManager animationManager;

     private void Start()
     {
         animationManager = GameObject.FindWithTag("AnimationManager").GetComponent<AnimationManager>();
     }

     public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        animationManager.ChangeScene("SampleScene");
    }
}
