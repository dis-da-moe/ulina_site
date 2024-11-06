using UnityEngine;
using TMPro;

public class SocialButtonManager : MonoBehaviour
{
    public Color32 textColour;
    public void Start()
    {
        TextMeshProUGUI TextObj = gameObject.transform.parent.GetChild(0).GetComponent<TextMeshProUGUI>();
        TextObj.text = URLopener.SocialUrl[gameObject.transform.parent.name].LinkName;
        TextObj.color = textColour;
        TextObj.fontStyle = FontStyles.Underline;
    }
    public void OnButtonClick()
    {
        AudioManager.ClickPlay();
        URLopener.openIt(URLopener.SocialUrl[gameObject.transform.parent.name].URL);
    }
}
