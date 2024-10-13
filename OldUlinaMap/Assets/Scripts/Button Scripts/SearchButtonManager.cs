using System;
using UnityEngine;
using TMPro;
public class SearchButtonManager : MonoBehaviour
{
    public AnimationManager animationManager;

    private void Start()
    {
        animationManager = GameObject.FindWithTag("AnimationManager").GetComponent<AnimationManager>();
    }

    public void OnButtonClick()
    {
        AudioManager.ClickPlay();

        string continent = gameObject.transform.parent.GetChild(1).GetComponent<TextMeshProUGUI>().text;
        string nation = gameObject.transform.parent.GetChild(0).GetComponent<TextMeshProUGUI>().text;
        animationManager.FindSearch(continent, nation);
    }
}
