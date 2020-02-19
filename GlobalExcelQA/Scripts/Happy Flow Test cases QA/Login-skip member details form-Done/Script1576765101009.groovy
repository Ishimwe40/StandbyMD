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

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Login-skip member details form-Done/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_Agree'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Skip'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/a_Reported'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_DOB_DOB'))

WebUI.sendKeys(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_DOB_DOB'), 
    '12142016')

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/img_Male_img-responsive'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Symptom Selector'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Next'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Next_1'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_Skin changes'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_Fever'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_Diarrhea'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Terms of Use_Address'), 
    'kigali')

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Between 1004 and 104 F (38 and 40 C)'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Less than 2 weeks'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Yes'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Yes'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/label_Yes'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Ok'))

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Member_Member'), 
    'Laetitia')

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/select_Test'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/select_Test_1'), 
    '11', true)

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Policy _Policy'), 
    '12345')

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Case _ClaimRef'), 
    '123456')

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    '0788509927')

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/div_Member                                 _88f63f'))

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Email_Email'), 
    'laetitia.i@muraho.tech')

WebUI.setText(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/input_Member_Member'), 
    'Laetitia')

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Save'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Select'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/span_Submit Request'))

WebUI.click(findTestObject('Object Repository/Login-skip member details form-Done/Page_StandbyMD Directional Care/button_Done'))

WebUI.closeBrowser()

