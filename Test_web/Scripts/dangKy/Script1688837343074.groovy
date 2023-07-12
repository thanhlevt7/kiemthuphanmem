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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('https://bio-ngon.com/account/register')

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_ng k bngs in thoi_customerlast_name'), 
    ho)

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_ng k bngs in thoi_customerfirst_name'), 
    ten)

WebUI.click(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/label_Nam'))

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_Nam_customerbirthday'), nsinh)

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_Nam_customeremail'), email)

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_Nam_customerphone'), sdt)

WebUI.setText(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_Nam_customerpassword'), password)

WebUI.click(findTestObject('Object Repository/dangKy/Page_To ti khon  Bio Ngon/input_Nam_btn'))

WebUI.verifyElementPresent(findTestObject('dangKy/Page_To ti khon  Bio Ngon/li_Chng ti  gi email n trandangmail.com, xin vui lng vo lin kt trong mail  xc minh'), 
    1)

WebUI.closeBrowser()

