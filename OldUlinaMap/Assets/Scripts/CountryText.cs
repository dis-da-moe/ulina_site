using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;
using UnityEngine.EventSystems;

public class CountryText : MonoBehaviour, IPointerClickHandler
{
    private bool disabled;
    public void OnPointerClick(PointerEventData eventData)
    {
        if (!disabled)
        {
            AudioManager.ClickPlay();
            StartCoroutine(DisableButton());
            GameObject.FindGameObjectWithTag("animator").GetComponent<ContentAnimationManager>().AnimateContentChange(gameObject.name);
        }
    }
    
    IEnumerator DisableButton()
    {
        disabled = true;
        yield return new WaitForSeconds(0.4f);
        disabled = false;
    }
}
