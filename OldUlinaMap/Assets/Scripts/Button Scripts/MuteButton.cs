using System.Collections;
using UnityEngine;
using UnityEngine.UI;

public class MuteButton : MonoBehaviour
{
    public Button Button;
    public Image Image;
    public Sprite Unmuted;
    public Sprite Muted;
    public string type;

    void Start()
    {
        Image.sprite = AudioManager.GetStatus(type) ? Unmuted : Muted;
    }
    public void OnButtonClick()
    {
        Image.sprite = AudioManager.GetStatus(type) ? Muted : Unmuted;
        AudioManager.AudioUpdate(type);
        StartCoroutine(ButtonDisable());
    }

    IEnumerator ButtonDisable()
    {
        Button.enabled = false;
        yield return new WaitForSeconds(0.3f);
        Button.enabled = true;
    }
}
