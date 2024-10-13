using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Networking;
using System.IO;

#if UNITY_EDITOR
using UnityEditor;
#endif

[ExecuteInEditMode] 
public class Editor : MonoBehaviour
{
    private const string url = "https://docs.google.com/spreadsheets/d/10PDI5edrdl0kLslvPzOkjRoP04KZD9pnylYSlL9voNU/export?format=csv";
#if UNITY_EDITOR

    void Awake()
    {
        if (!Application.isPlaying)
        {
            StartCoroutine(DownloadData(OnDownload));
        }


    }

    void OnDownload(string download)
    {
        if (download != null)
        {
            using (FileStream fs = new FileStream("Assets/Resources/sheet.csv", FileMode.Create))
            {
                using (StreamWriter writer = new StreamWriter(fs))
                {
                    writer.Write(download);
                }
            }
            AssetDatabase.Refresh();
            Debug.Log("saved");
            NationDataInit init = GameObject.Find("NationData").GetComponent<NationDataInit>();
            init.CheckFlags();
        }
    }

    internal static IEnumerator DownloadData(System.Action<string> onCompleted)
    {
        string download;
        
        using (UnityWebRequest request = UnityWebRequest.Get(url))
        {
            yield return request.SendWebRequest();

            if (request.isNetworkError)
            {
                Debug.LogError("Failed Download");
            }
            else
            {
                Debug.Log("Downloaded");
                
            }

            download = request.downloadHandler.text;
        }

        onCompleted(download);
    }
#endif
}
