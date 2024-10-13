using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;

public class ContentCreator : MonoBehaviour
{
    #region template variables
    public GameObject InstaButtonTemplate;
    public GameObject WikiButtonTemplate;
    public GameObject TitleTemplate;
    public GameObject DescriptionTemplate;
    public GameObject FlagTemplate;
    public GameObject SiteTemplate;
    #endregion

    #region other variables

    public List<GameObject> Contents;
    public GameObject[] ContentObjects;

    public nation Nation;
    private string Name;
    #endregion
    public void PopulateContent(string name, nation nation,GameObject Content)
    {
        DepopulateContent();
        
        CreateElement(TitleTemplate, 0, name);

        Nation = nation;
        Name = name;

        CheckFlag();
        CheckSocials();
        CheckDescription();
        CheckExtra();
        LayoutRebuilder.ForceRebuildLayoutImmediate((RectTransform)Content.transform);
    }
    public void CreateElement(GameObject template, int type, string text)
    {
        GameObject Element = Instantiate(template) as GameObject;
        Contents.Add(Element);
        Element.transform.SetParent(template.transform.parent, false);
        Element.tag = "Content";
        Element.SetActive(true);
        switch(type)
        {
            case 0:
            Element.GetComponent<TextMeshProUGUI>().text = text;
            break;
            case 1:
            Element.transform.GetChild(0).GetComponent<InstaButton>().insta = text;
            break;
            case 2:
            Element.transform.GetChild(0).GetComponent<WikiButton>().wiki = text;
            break;
            case 3:
            Element.GetComponent<TextMeshProUGUI>().text = text;
            StartCoroutine(UpdateDescriptionLayout(Element));
            break;
            case 4:
            Element.GetComponent<Image>().sprite = NationData.Flags.FlagsNames[text];
            Element.transform.SetAsFirstSibling();
            break;
            case 5:
            Element.transform.GetChild(0).GetComponent<SiteButton>().updateText(text);
            break;
        }
    }

    public IEnumerator UpdateDescriptionLayout(GameObject description)
    {
        description.GetComponent<ContentSizeFitter>().enabled = true;
        yield return new WaitForSeconds(0);
        description.GetComponent<ContentSizeFitter>().enabled = false;
    }
    public void DepopulateContent()
    {
        ContentObjects = GameObject.FindGameObjectsWithTag("Content");
        foreach (GameObject Object in ContentObjects)
        {
            Destroy(Object);
        }
    }

#region ContentChecks
    public void CheckSocials()
    {
        if (Nation.insta != "")
        {
            CreateElement(InstaButtonTemplate, 1, Nation.insta);
        }
        
        if (Nation.wiki != "")
        {
            CreateElement(WikiButtonTemplate, 2, Nation.wiki);
        }
    }
    public void CheckDescription()
    {
        if (Nation.description != "")
        {
            CreateElement(DescriptionTemplate,3, Nation.description);
            
        }
        if (Nation.description == "")
        {
            CreateElement(DescriptionTemplate,3, $"Welcome to {Name}!");
        }
    }
    public void CheckFlag()
    {
        if (NationData.Flags.FlagsNames.ContainsKey(Name))
        {
            CreateElement(FlagTemplate, 4, Name);
        }
    }
    public void CheckExtra()
    {
        if(Nation.extra != null)
        {
            CreateElement(SiteTemplate,5, Nation.extra.name + "\n" + Nation.extra.url);
        }
    }
#endregion
}
