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

WebUI.click(findTestObject('Page_Choose your account/button_Client Sign-in'))

WebUI.setText(findTestObject('Page_Sign in to your account/input_concat(Sorry but we  re having troubl_5aa0e7'), 'qa3@globalexcel.com')

WebUI.setEncryptedText(findTestObject('Page_Sign in to your account/input_Redirecting_passwd'), 'axFQsTjMQqya7oMWgLoxVA==')

WebUI.click(findTestObject('Page_Sign in to your account/button_Sign in'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/div_No'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/div_Agree'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Skip'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/input_DOB_DOB'))

WebUI.sendKeys(findTestObject('Page_StandbyMD Directional Care/input_DOB_DOB'), '12121988')

WebUI.click(findTestObject('Page_StandbyMD Directional Care/img_Male_img-responsive'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Pregnancy'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/label_Symptom Selector'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Next'))

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Terms of Use_Address'), 'kigali')

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Next_1'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Sore throat'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Skin changes'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Runny nose'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Nonmenstrual vaginal bleeding or spotting'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Headache'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Fever'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Headache'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Next_1_2'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/div_Chills'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Next_1_2_3'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/label_Greater than 104 F (40 C)'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/label_II trimester - the second trimester i_0be234'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/label_No'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/label_Dont know'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/label_Dont know'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Ok'))

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Member_Member'), 'sdf fge45657')

WebUI.selectOptionByValue(findTestObject('Page_StandbyMD Directional Care/select_Test'), '4', true)

WebUI.selectOptionByValue(findTestObject('Page_StandbyMD Directional Care/select_Test_1'), '11', true)

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Policy _Policy'), 'retrdf5668')

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Case _ClaimRef'), 'dfdgtr557')

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Phone_Phone'), 'dgryt56567')

WebUI.setText(findTestObject('Page_StandbyMD Directional Care/input_Email_Email'), 'ffghg67666666jyt')

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Save'))

WebUI.click(findTestObject('Page_StandbyMD Directional Care/button_Select'))

WebUI.click(findTestObject('Object Repository/Page_StandbyMD Directional Care/a_Triage'))

WebUI.delay(2)

WebUI.closeBrowser()

