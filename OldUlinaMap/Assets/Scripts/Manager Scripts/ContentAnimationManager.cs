using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;

public class ContentAnimationManager : MonoBehaviour
{
    Animator transition;
    public GameObject Content;

    private void Awake()
    {
        transition = GetComponentInChildren<Animator>();
    }

    public void AnimateContentChange(string NationName)
    {
        StartCoroutine(ContentAnimate(NationName));

    }

    IEnumerator ContentAnimate(string NationName)
    {
        Scenemanager.SceneChange = false;
        transition.SetTrigger("FadeIn");

        yield return new WaitForSeconds(0.4f);
        
        if (NationName != "")
        {
            NationData.CountryClicked(Content, NationName);
            Content.GetComponent<VerticalLayoutGroup>().enabled = false;
            yield return new WaitForSeconds(0);
            Content.GetComponent<VerticalLayoutGroup>().enabled = true;
        }
        else 
        {
            NationData.CloseContent(Content);
        }
        yield return new WaitForSeconds(0.1f);
        transition.SetTrigger("FadeOut");
        Scenemanager.SceneChange = true;
    }
}
