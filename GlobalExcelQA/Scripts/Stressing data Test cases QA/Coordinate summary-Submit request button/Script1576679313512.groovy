import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://standbymd-qa3.globalexcel.com/#')

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Coordinate-Submit request/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Agree'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Skip'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_DOB_DOB'))

WebUI.sendKeys(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_DOB_DOB'), 
    '12121988')

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/span_Male'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_Symptom Selector'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Next'))

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Terms of Use_Address'), 
    'kigali')

WebUI.click(findTestObject('Coordinate-Submit request/Page_StandbyMD Directional Care/button_Next_1'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Feeling sick'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Fever'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Headache'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Cough'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Back pain'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Muscle pain'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Generalized headache'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/div_Diminished appetite'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Next_1_2_3'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_I havent checked my temperature'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_Mild'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_Yes'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_Less than 3 weeks'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/label_Less than 3 months'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Ok'))

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Member_Member'), 
    'DDGFHG56578789%^&**(')

WebUI.selectOptionByValue(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/select_Test'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/select_Test_1'), 
    '11', true)

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Policy _Policy'), 
    'DFTYRY5667')

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Case _ClaimRef'), 
    'DGFGHFR565675')

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    'DFDJGORJ5557')

WebUI.setText(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/input_Email_Email'), 
    'NDFKNDKNFGK34U96&*(()_)_')

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Save'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Select'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/a_Contact'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/a_Service'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/a_Triage'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/i_Coordinate Summary_fas fa-sync-alt fa-2x'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/i_Coordinate Summary_far fa-clipboard fa-2x'))

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/span_Submit Request'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Coordinate-Submit request/Page_StandbyMD Directional Care/button_Done'))

WebUI.closeBrowser()

