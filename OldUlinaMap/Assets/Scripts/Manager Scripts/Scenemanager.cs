using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class Scenemanager : MonoBehaviour
{
    void Awake()
    {
        SceneManager.LoadScene(1, LoadSceneMode.Additive);
    }

    static public bool SceneChange = true;
}
