using UnityEngine;
using TMPro;

public class SiteButton : MonoBehaviour
{
    string[] list;

    public void updateText(string site)
    {
        list = site.Split();
        transform.parent.GetChild(1).GetComponent<TextMeshProUGUI>().text = list[0];
    }

    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        URLopener.openIt(list[1]);
    }
}
