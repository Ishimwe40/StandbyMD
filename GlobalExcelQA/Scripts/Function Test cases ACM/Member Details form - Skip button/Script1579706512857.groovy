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

WebUI.click(findTestObject('Object Repository/Member details form- Skip button/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'acm3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Member details form- Skip button/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Member details form- Skip button/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/div_No'))

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/input_Member_Member'), 
    'Laetitia')

WebUI.selectOptionByValue(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/select_CF Services (TII)LloydsManulifeOrion_fb81e2'), 
    '14', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/select_AMA                    Atlantic     _521c83'), 
    '47', true)

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/input_Policy _Policy'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/input_Case _ClaimRef'), 
    '1')

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/input_Phone_Phone'), 
    '0788509927')

WebUI.setText(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/input_Email_Email'), 
    'laetitia.i@muraho.tech')

WebUI.click(findTestObject('Object Repository/Member details form- Skip button/Page_StandbyMD Directional Care/button_Skip'))

WebUI.delay(2)

WebUI.closeBrowser()

