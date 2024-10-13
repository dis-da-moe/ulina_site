using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using TMPro;
public class colourButton : MonoBehaviour
{
    public Color32 ButtonColour;
    public static Color32 ButtonColourStatic;
    public Color ButtonHighlightedColour;
    public static Color32 ButtonHighlightedColourStatic;
    
    void Awake()
    {
        ButtonColourStatic = ButtonColour;
        ButtonHighlightedColourStatic = ButtonHighlightedColour;
    }
}
