using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class AnimationManager : MonoBehaviour
{
    public Animator transition;
    public void ChangeScene(string LoadTo)
    {
        if (Scenemanager.SceneChange)
        {
            StartCoroutine(LoadAnimation(LoadTo));
        }
    }
    IEnumerator LoadAnimation(string LoadTo)
    {
        Scenemanager.SceneChange = false;
        transition.Play("CrossFadeEnd");
        yield return new WaitForSeconds(1.2f);
        SceneManager.LoadScene(LoadTo, LoadSceneMode.Additive);
        SceneManager.UnloadSceneAsync(gameObject.scene.name);
        Scenemanager.SceneChange = true;
    }
    public void FindSearch(string LoadTo, string nation)
    {
        if (Scenemanager.SceneChange)
        {
            StartCoroutine(LoadSearch(LoadTo, nation));
        }
    }
    IEnumerator LoadSearch(string LoadTo, string nation)
    {
        Scenemanager.SceneChange = false;
        transition.Play("CrossFadeEnd");
        yield return new WaitForSeconds(1.2f);
        SceneManager.LoadScene(LoadTo, LoadSceneMode.Additive);
        yield return new WaitForSeconds(0.2f);
        NationData.OpenSearch(nation);
        SceneManager.UnloadSceneAsync(gameObject.scene.name);
        Scenemanager.SceneChange = true;
    }
}
