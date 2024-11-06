using UnityEngine;

public class WikiButton : MonoBehaviour
{
    public string wiki;
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        URLopener.openIt(wiki);
    }
}
