using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class AudioManager : MonoBehaviour
{
    static public bool sfxOn = true;
    static public bool MusicOn = true;
    static AudioSource Music;
    static AudioSource Click;
    void Awake()
    {
        Music = gameObject.transform.GetChild(0).GetComponent<AudioSource>();
        Click = gameObject.transform.GetChild(1).GetComponent<AudioSource>();
        Music.Play();
    }

    static public void AudioUpdate(string type)
    {
        if (type == "sfx")
        {
            //Click.transform.parent.gameObject.SetActive(true);

            sfxOn = !sfxOn;
        }
        else
        {
            if (MusicOn)
            {
                Music.Pause();
                MusicOn = false;
            }
            else
            {
                Music.Play();
                
                MusicOn = true;
            }
        }


    }
    static public void ClickPlay()
    {
        if (sfxOn)
        {
            Click.PlayOneShot(Click.clip);
        }
    }

    static public bool GetStatus(string type)
    {
        if (type == "sfx")
        {
            return sfxOn;
        }
        else
        {
            return MusicOn;
        }
    }
}
