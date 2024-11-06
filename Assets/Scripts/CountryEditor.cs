using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using TMPro;
using Unity.Mathematics;

#if UNITY_EDITOR
using UnityEditor;
#endif

[ExecuteInEditMode]
public class CountryEditor : MonoBehaviour
{
#if UNITY_EDITOR

    private NationDataInit init;
    public GameObject content;
    public GameObject template;
    List<string> instantiated;
    public TMP_FontAsset small;
    private float pos;

    private void Awake()
    {
        init = GameObject.Find("NationData").GetComponent<NationDataInit>();
        init.Load();

        pos = 0;
        
        instantiated = new List<string>();
        for (int i = 0; i < transform.childCount; i++)
        {
            instantiated.Add(transform.GetChild(i).name);
        }

        if (!Application.isPlaying)
        {
            List<string> names = NationData.Nations.Keys.ToList();
            for (int i = 0; i < NationData.Nations.Count; i++)
            {
                string countryName = names[i];

                if (NationData.Nations[countryName].continent == gameObject.transform.parent.name)
                {
                    if (!instantiated.Contains(countryName))
                    {
                        GameObject country = Instantiate(template, new Vector2(50, pos), Quaternion.identity, transform);
                        country.name = countryName;
                        
                        TextMeshProUGUI textMeshPro = country.GetComponent<TextMeshProUGUI>();
                        countryName = removeBrackets(countryName, textMeshPro);
                        country.GetComponent<TextMeshProUGUI>().text = countryName;
                        
                        pos += 40;
                    }
                }
            }
            AssetDatabase.SaveAssets();
        }

    }

    string removeBrackets(string countryName, TextMeshProUGUI textMeshPro = null)
    {
        if (countryName.Contains("("))
        {
            int start = countryName.IndexOf('(');
            int end = countryName.IndexOf(')');

            countryName = countryName.Substring(0,start);

            if (textMeshPro)
            {
                textMeshPro.fontStyle = FontStyles.Italic;
            }
        }

        return countryName;
    }
#endif

}
