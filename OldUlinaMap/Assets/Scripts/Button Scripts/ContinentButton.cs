using UnityEngine;
using UnityEngine.UI;

public class ContinentButton : MonoBehaviour{
    public GameObject AnimationManager;

    void Awake()
    {
        ColorBlock buttonColors = gameObject.transform.parent.GetComponent<Button>().colors;
        buttonColors.highlightedColor = colourButton.ButtonHighlightedColourStatic;
        gameObject.transform.parent.GetComponent<Button>().colors = buttonColors;
    }
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        string toLoad = gameObject.transform.parent.gameObject.name;
        AnimationManager.GetComponent<AnimationManager>().ChangeScene(toLoad);
    }
}