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

WebUI.navigateToUrl('https://standbymd-qa3.globalexcel.com')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'W1+dqQw4nbLuXNwGTywvuw==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    '2I45U+vLq5ev6oiqySW8OA==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'RxWUgsovFP03c5HqaKjJRw==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'VI0Sq04/JFZRMPWCBQItfg==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'd/aXD7yjD+4=')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'gL1PUrrlpLJ95+8GTmgHvw==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'lApfWctp3m56BbyDfNlOLg==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'h/cKTk+88NVgbxQh6YeGwQ==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'uVN5TceFCRaqLUdjF257uA==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.setEncryptedText(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/input_Redirecting_passwd'), 
    'yGsmk8nnrnubi0M8g07t5A==')

WebUI.click(findTestObject('Object Repository/Login-How many time to handle invalid password/Page_Sign in to your account/button_Sign in'))

WebUI.closeBrowser()

