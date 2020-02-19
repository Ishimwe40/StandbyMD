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

WebUI.navigateToUrl('https://standbymd-qa3.globalexcel.com/')

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'acm3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Contacts-Edit icon/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/button_Skip'))

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/i_Contacts_fas fa-plus-circle'))

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Yes_opCaller'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/select_MrMrsMsDr'), 
    '5', true)

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Name_FirstName'), 
    'Laetitia')

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    '0788509927')

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Address_Address'), 
    'kigali')

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_E-Mail_EMail'), 
    'laetitia.i@muraho.tech')

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/button_Save changes'))

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/i_Contacts_fas fa-plus-circle'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/select_PatientNonPatient'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/select_MrMrsMsDr'), 
    '4', true)

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Name_FirstName'), 
    'Selina')

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_E-Mail_EMail'), 
    'selina@muraho.tech')

WebUI.setText(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/input_Address_Address'), 
    'harare')

WebUI.click(findTestObject('Object Repository/Contacts-Edit icon/Page_StandbyMD Directional Care/button_Save changes'))

WebUI.delay(2)

WebUI.closeBrowser()

