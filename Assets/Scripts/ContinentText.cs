using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.EventSystems;

public class ContinentText : MonoBehaviour, IPointerClickHandler
{
    private bool disabled;
    public GameObject AnimationManager;
    public void OnPointerClick(PointerEventData eventData)
    {
        if (!disabled)
        {
            AudioManager.ClickPlay();
            StartCoroutine(DisableButton());
            string toLoad = name;
            AnimationManager.GetComponent<AnimationManager>().ChangeScene(toLoad);
        }
    }
    
    IEnumerator DisableButton()
    {
        disabled = true;
        yield return new WaitForSeconds(0.4f);
        disabled = false;
    }
}
