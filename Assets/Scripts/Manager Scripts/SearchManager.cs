using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using TMPro;

public class SearchManager : MonoBehaviour
{
    public GameObject CountrySearchTemplate;
    public List<GameObject> searchResults = new List<GameObject>();
    string SearchedBefore;
    public void SearchCountry(string CountryToSearch)
    {
        if (CountryToSearch != "")
        {
            if (CountryToSearch != SearchedBefore)
            {
                DestoryResults();
                foreach (KeyValuePair<string, nation> Pair in NationData.Nations)
                {
                    if (Pair.Key.ToLower().Contains(CountryToSearch.ToLower()))
                    {
                        CreateSearchButton(Pair.Key);
                       
                    }
                }
                SearchedBefore = CountryToSearch;
            }

        }
        else
        {
            DestoryResults();
        }
    }

    public void CreateSearchButton(string NationName)
    {
        GameObject CountrySearch = Instantiate(CountrySearchTemplate, CountrySearchTemplate.transform.parent, false);
        CountrySearch.SetActive(true);
        CountrySearch.transform.GetChild(0).GetComponent<TextMeshProUGUI>().text = NationName;
        CountrySearch.transform.GetChild(1).GetComponent<TextMeshProUGUI>().text = Capatalise(NationData.Nations[NationName].continent);
        searchResults.Add(CountrySearch);
    }

    public void DestoryResults()
    {
        foreach (GameObject Result in searchResults)
        {
            Destroy(Result);            
        }
        searchResults.Clear();
    }

    public string Capatalise(string String)
    {
        if (string.IsNullOrEmpty(String))
        {
            return string.Empty;
        }
        return char.ToUpper(String[0]) + String.Substring(1);
    }
}
