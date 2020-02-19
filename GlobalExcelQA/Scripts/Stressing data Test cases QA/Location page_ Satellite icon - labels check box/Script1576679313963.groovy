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

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Satelite_label check box/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Satelite_label check box/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_StandbyMD Directional Care/div_Yes'))

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_StandbyMD Directional Care/div_Yes'))

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_StandbyMD Directional Care/div_Satellite'))

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_StandbyMD Directional Care/img'))

WebUI.click(findTestObject('Object Repository/Satelite_label check box/Page_StandbyMD Directional Care/img_1'))

WebUI.closeBrowser()

