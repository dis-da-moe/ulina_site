using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System.Runtime.InteropServices;

public class URLopener : MonoBehaviour
{
    public class social
    {
        public string URL;
        public string LinkName;
        public social(string URL, string LinkName)
        {
            this.URL = URL;
            this.LinkName = LinkName;
        }
    }
    public static Dictionary <string, social> SocialUrl = new Dictionary<string, social>()
    {
        {"OfficialDiscord", new social("https://discord.gg/dZGsaGmeJQ", "Discord")},
        {"OfficialInsta", new social("https://www.instagram.com/ulinaworld/", "Instagram")},
        {"OfficialWiki", new social("https://ulina.fandom.com/wiki/Ulina_Wiki", "Wikipedia")},
        {"Zok", new social("https://www.instagram.com/zoknic/", "Instagram")},
        {"Vincent", new social("https://worms3401.bandcamp.com/", "Bandcamp")},
        {"Moe", new social("https://www.instagram.com/uraj_ah/", "Instagram")},
        {"OfficialSite", new social("https://www.ulinaworld.com/", "Website")}
    };
    [DllImport("__Internal")]
    private static extern void OpenNewTab(string url);

    static public void openIt(string url)
    {
#if !UNITY_EDITOR && UNITY_WEBGL
             OpenNewTab(url);
#endif
    }
}
