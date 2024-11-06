using System.Collections;
using UnityEngine;
using UnityEngine.UI;

public class CountryButton : MonoBehaviour
{
    public string NationName;

    void Awake()
    {
        #region Button Colours
        ColorBlock buttoncolors = gameObject.transform.parent.GetComponent<Button>().colors;
        buttoncolors.highlightedColor = colourButton.ButtonHighlightedColourStatic;
        gameObject.transform.parent.GetComponent<Button>().colors = buttoncolors;
        gameObject.transform.parent.GetComponent<Button>().image.color = colourButton.ButtonColourStatic;
        #endregion
    }
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        StartCoroutine(DisableButton(gameObject.transform.parent.gameObject));
        NationName = gameObject.name;
        GameObject.FindGameObjectWithTag("animator").GetComponent<ContentAnimationManager>().AnimateContentChange(NationName);
    }
    IEnumerator DisableButton(GameObject Button)
    {
        Button.GetComponent<Button>().enabled = false;
        yield return new WaitForSeconds(0.4f);
        Button.GetComponent<Button>().enabled = true;
    }
}
