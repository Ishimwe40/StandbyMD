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

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_Choose your account/button_Client Sign-in'))

WebUI.maximizeWindow()

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'acm3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Interview-Next button/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/div_Agree'))

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_Member_Member'), 
    'Laetitia Ishimwe')

WebUI.selectOptionByValue(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/select_Test'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/select_Test_1'), 
    '11', true)

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_Policy _Policy'), 
    '25')

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_Case _ClaimRef'), 
    '12')

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    '0788509927')

WebUI.setText(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_Email_Email'), 
    'laetitia.i@muraho.tech')

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Save'))

WebUI.sendKeys(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/input_DOB_DOB'), 
    '12121988')

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/img_Gender_img-responsive'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Symptom Selector'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next_1'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/div_Back pain'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_No'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/div_Stomach pain'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next_1_2_3'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Moderate'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Yes'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Upper right'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Upper middle'))

WebUI.delay(2)

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/button_Next_1_2_3_4'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_2 - 7 days'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_No'))

WebUI.click(findTestObject('Object Repository/Interview-Next button/Page_StandbyMD Directional Care/label_Yes'))

WebUI.closeBrowser()

