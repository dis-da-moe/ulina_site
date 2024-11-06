using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class NationDataInit : MonoBehaviour
{
    flags Flags;
    TextAsset sheet;
    
    void Start()
    {
        Load();
    }

    public void Load()
    {
        NationData.Nations.Clear();
        sheet = Resources.Load<TextAsset>("sheet");
        fgCSVReader.LoadFromString(sheet.text, MyReader);

        Flags = new flags();
        Flags.CreateFlags();
        NationData.Flags = Flags;
    }

    public void CheckFlags()
    {
        if (NationData.Flags == null || NationData.Nations == null)
        {
            Load();
        }
        List<string> flagNames = NationData.Flags.FlagsNames.Keys.ToList();
        List<string> nationNames = NationData.Nations.Keys.ToList();

        List<string> flagButNoNation = flagNames.Except(nationNames).ToList();
        for (int i = 0; i < flagButNoNation.Count; i++)
        {
           Debug.Log($"Flag {flagButNoNation[i]} is present in flag folder but there is no nation"); 
        }

        List<string> nationButNoFlag = nationNames.Except(flagNames).ToList();
        for (int i = 0; i < nationButNoFlag.Count; i++)
        {
            Debug.Log($"Nation {nationButNoFlag[i]} is present but no flag is there in the folder");
        }


        
    }

    void MyReader(int line_index, List<string> line)
    {

        if(line_index > 1 )
        {
            site extra = line[6].Contains("https") ? new site(line[5], line[6]) : null;
            NationData.Nations.Add(line[0], new nation(line[2], line[3], line[4], line[1].ToLower(), extra));
        }
    }
}
