using UnityEngine;

public class InstaButton : MonoBehaviour
{
    public string insta;
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        URLopener.openIt(insta);
    }
}
