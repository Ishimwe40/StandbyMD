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

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Object Repository/Login-No-End call-Submit button/Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 
    'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Login-No-End call-Submit button/Page_Sign in to your account/input_Redirecting_passwd'), 
    'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/div_Disagree'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/div_Yes'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/div_Yes'))

WebUI.setText(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/input_Terms of Use_Address'), 
    'kigali')

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/button_Next'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/span_King Faisal Hospital'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/div_Distance'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/i_Murambi Hospital_flagFacility'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/select_Incorrect AddressIncorrect HoursInco_e9a202'), 
    '7', true)

WebUI.setText(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/textarea_Comments_Comments'), 
    'health')

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/button_Submit'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/button_Yes'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/button_Ok'))

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/i_Kigali Kigali City_fas fa-2x fa-phone-slash'))

WebUI.selectOptionByValue(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/select_Call was interruptedEmergency situat_490c73'), 
    '11', true)

WebUI.click(findTestObject('Object Repository/Login-No-End call-Submit button/Page_StandbyMD Directional Care/button_Submit'))

WebUI.delay(2)

WebUI.closeBrowser()

