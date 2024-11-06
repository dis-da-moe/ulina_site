using System.Collections.Generic;
using UnityEngine;

public static class NationData
{
    #region Variables
    public static flags Flags;
    static string CurrentNation = "";
    public static Dictionary <string, nation> Nations = new Dictionary<string, nation>();
    #endregion

    #region country and content functions
    public static void CountryClicked(GameObject Content, string NationName)
    {
        if (Content.activeSelf && (NationName == CurrentNation))
        {
            CloseContent(Content);
            CurrentNation = "";
        }
        else
        {
            OpenContentNation(Content, NationName);
            CurrentNation = NationName;
        }
    }

    static void OpenContentNation(GameObject Content, string Nation)
    {
        Content.SetActive(true);
        nation nation = Nations[Nation];
        Content.GetComponent<ContentCreator>().PopulateContent(Nation, nation ,Content);
    }
    public static  void CloseContent(GameObject Content)
    {
        Content.GetComponent<ContentCreator>().DepopulateContent();
        Content.SetActive(false);
    }
    public static void OpenSearch(string nation)
    {
        GameObject Animator = GameObject.FindWithTag("animator");
        Animator.GetComponent<ContentAnimationManager>().AnimateContentChange(nation);
    }
#endregion
}

#region classes
public class flags
{
    public IDictionary<string, Sprite> FlagsNames = new Dictionary<string, Sprite>();
    public void CreateFlags()
    {
        Sprite[] flagSprites = Resources.LoadAll<Sprite>("Flags");
        for (int i = 0; i < flagSprites.Length; i++)
        {
            FlagsNames.Add(flagSprites[i].name, flagSprites[i]);
        }
    }
}

public class nation
{
    public string insta;
    public string wiki;
    public string description;
    public string continent;
    public site extra;
    public nation(string insta, string wiki, string description, string continent, site extra=null)
    {
        this.insta = insta;
        this.wiki = wiki;
        this.description = description; 
        this.continent = continent;
        if(extra != null)
        {
            this.extra = extra;
        }
    }
}
public class site
{
    public string name;
    public string url;
    public site(string name, string url)
    {
        this.name = name;
        this.url = url;
    }
}

#endregion