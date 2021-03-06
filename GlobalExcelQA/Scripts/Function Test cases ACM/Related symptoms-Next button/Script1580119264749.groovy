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

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Related symptoms-Next button/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'acm3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Related symptoms-Next button/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/div_Agree'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_Skip'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/input_DOB_DOB'))

WebUI.sendKeys(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/input_DOB_DOB'), 
    '12121988')

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/img_Gender_img-responsive'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/label_NLP (English Only)'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_Next'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_Next_1'))

WebUI.setText(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/textarea_Describe the members symptoms or p_874eed'), 
    'Headache\nFever')

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/div_Headache'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/div_Fever'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button__1'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button__1'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button__1'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Object Repository/Related symptoms-Next button/Page_StandbyMD Directional Care/button_Next_1_2_3'))

WebUI.delay(2)

WebUI.closeBrowser()

