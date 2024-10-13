using UnityEngine;
using UnityEngine.UI;

public class CloseButton : MonoBehaviour
{
    public Button button;
    void OnEnable()
    {
        button.enabled = true;
    }
    public void OnButtonClick()
    {
        button.enabled = false;
        AudioManager.ClickPlay();
        GameObject.FindGameObjectWithTag("animator").GetComponent<ContentAnimationManager>().AnimateContentChange("");
    }
}
