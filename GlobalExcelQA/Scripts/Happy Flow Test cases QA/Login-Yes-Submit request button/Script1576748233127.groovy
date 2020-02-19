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

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Login-YES-Submit request button/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/div_Yes'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/div_Yes'))

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Terms of Use_Address'), 
    'kigali')

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/div_Map'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/img'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/div_Satellite'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Terms of Use_gm-control-active gm-fu_309b32'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Report a map error_gm-control-active'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Report a map error_gm-control-active_1'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Terms of Use_gm-control-active gm-fu_309b32'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Next'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/span_King Faisal Hospital'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/div_Distance'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Next_1'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/i_Contacts_fas fa-plus-circle'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_PatientNonPatient'), 
    '2', true)

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Yes_opCaller'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_MrMrsMsDr'), 
    '3', true)

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Name_FirstName'), 
    'Laetitia')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    '0788509927')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Mobile_Mobile'), 
    '23435465')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Fax_Fax'), 
    '4544')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_E-Mail_EMail'), 
    'laetitia.i@muraho.tech')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Address_Address'), 
    'kigali')

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_Afghanistanland IslandsAlbaniaAlgeri_e5d3ee'), 
    'RW', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_Prfecture de Butare  Prfecture de By_8a0cdc'), 
    '09', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_Bandazi  Bicumbi  Bihembe  Bugaragar_989edf'), 
    '9006568', true)

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Attention_Attention'), 
    'hi')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/textarea_Remark_Remark'), 
    'hello')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Reference_Reference'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Postal Code_Zip'), 
    '23')

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Save changes'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/a_Contact'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/a_Service'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/a_Triage'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/i_Coordinate Summary_fas fa-sync-alt fa-2x'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/i_Coordinate Summary_far fa-clipboard fa-2x'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/span_Submit Request'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Ok'))

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Member_Member'), 
    'Laetitia')

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_Test'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/select_Test_1'), 
    '11', true)

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Policy _Policy'), 
    '12345')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Case _ClaimRef'), 
    '123456')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Phone_Phone_1'), 
    '0788509927')

WebUI.setText(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/input_Email_Email'), 
    'laetitia.i@muraho.tech')

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/button_Save'))

WebUI.click(findTestObject('Object Repository/Login-YES-Submit request button/Page_StandbyMD Directional Care/span_Submit Request'))

WebUI.closeBrowser()

